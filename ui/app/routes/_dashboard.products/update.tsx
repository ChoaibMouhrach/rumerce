import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { CategoriesInput } from "~/components/categories-input";
import { Button } from "~/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "~/components/ui/form";
import { Textarea } from "~/components/ui/textarea";
import { Input } from "~/components/ui/input";
import { UnitsInput } from "~/components/units-input";
import { storeProductSchema } from "~/validations/product";
import { Variants } from "./variants";
import { productService } from "~/services/products";
import { toast } from "sonner";
import React, { useState } from "react";
import { useRevalidator } from "@remix-run/react";
import { DropZone } from "~/components/drop-zone";
import { z } from "zod";
import { imageService } from "~/services/images";

const schema = storeProductSchema.extend({
  images: z
    .array(z.union([z.instanceof(File), z.string().includes(".")]))
    .min(1),
});

type Payload = z.infer<typeof schema>;

interface UpdateProps {
  product: Awaited<ReturnType<typeof productService.all>>[number];
}

export const Update: React.FC<UpdateProps> = ({ product }) => {
  const revalidator = useRevalidator();
  const form = useForm<Payload>({
    resolver: zodResolver(schema),
    values: {
      name: product.product.name,
      description: product.product.description,
      unit_id: product.unit.id,
      category_id: product.category.id,
      images: product.images.map((image) => image.src),
      variants: product.variants.map((variant) => ({
        ...variant.variant,
        checked: true,
        options: variant.collections.map((collection) => ({
          key: collection.key.name,
          value: collection.value.name,
        })),
      })),
    },
  });

  const [isPending, setIsPending] = useState(false);

  const onSubmit = async (payload: Payload) => {
    setIsPending(true);

    try {
      const files = payload.images.filter((image) => typeof image !== "string");

      const images = await Promise.all(
        files.map((file) => imageService.upload(file))
      );

      await productService.update(product.product.id, {
        ...payload,
        images: [
          ...payload.images
            .filter((image) => typeof image === "string")
            .map(
              (image) =>
                product.images.find(
                  (productImage) => productImage.src === image
                )!.id
            ),
          ...images.map((image) => image.id),
        ],
        variants: payload.variants.filter(({ checked }) => checked),
      });

      toast.success("Product updated successfully");
      form.reset();
      revalidator.revalidate();
    } catch {
      toast.error("Something went wrong");
    }

    setIsPending(false);
  };

  return (
    <Form {...form}>
      <form
        className="flex flex-col gap-4"
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <FormField
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormControl>
                <Input {...field} placeholder="Dell xps" />
              </FormControl>
              <FormDescription>The name of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name="category_id"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Category</FormLabel>
              <FormControl>
                <CategoriesInput
                  value={field.value}
                  onValueChange={field.onChange}
                />
              </FormControl>
              <FormDescription>The category of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name="unit_id"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Unit</FormLabel>
              <FormControl>
                <UnitsInput
                  value={field.value}
                  onValueChange={field.onChange}
                />
              </FormControl>
              <FormDescription>The unit of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name="description"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Description</FormLabel>
              <FormControl>
                <Textarea
                  {...field}
                  rows={8}
                  placeholder="This is a powerf..."
                />
              </FormControl>
              <FormDescription>The description of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name="images"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Images</FormLabel>
              <FormControl>
                <DropZone onValueChange={field.onChange} value={field.value} />
              </FormControl>
              <FormDescription>The images of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name="variants"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Variants</FormLabel>
              <FormControl>
                <Variants onValueChange={field.onChange} value={field.value} />
              </FormControl>
              <FormDescription>The variants of the product.</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button className="w-fit" type="submit" isPending={isPending}>
          Update
        </Button>
      </form>
    </Form>
  );
};
