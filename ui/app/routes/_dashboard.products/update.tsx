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
import { StoreProductPayload, storeProductSchema } from "~/validations/product";
import { Variants } from "./variants";
import { useMutation } from "@tanstack/react-query";
import { productService } from "~/services/products";
import { toast } from "sonner";
import React from "react";
import { useRevalidator } from "@remix-run/react";

type Payload = StoreProductPayload;

interface UpdateProps {
  product: Awaited<ReturnType<typeof productService.all>>[number];
}

export const Update: React.FC<UpdateProps> = ({ product }) => {
  const revalidator = useRevalidator();
  const form = useForm<Payload>({
    resolver: zodResolver(storeProductSchema),
    values: {
      name: product.product.name,
      description: product.product.description,
      unit_id: product.unit.id,
      category_id: product.category.id,
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

  const { mutate, isPending } = useMutation({
    mutationFn: (payload: Payload) => {
      return productService.update(product.product.id, payload);
    },
    onSuccess: () => {
      toast.success("Product updated successfully");
      form.reset();
      revalidator.revalidate();
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const onSubmit = (payload: Payload) => {
    mutate({
      ...payload,
      variants: payload.variants.filter(({ checked }) => checked),
    });
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
