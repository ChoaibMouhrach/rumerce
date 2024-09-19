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
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "~/components/ui/sheet";
import { UnitsInput } from "~/components/units-input";
import { StoreProductPayload, storeProductSchema } from "~/validations/product";
import { Variants } from "./variants";
import { productService } from "~/services/products";
import { toast } from "sonner";
import { useRevalidator } from "@remix-run/react";
import { DropZone } from "~/components/drop-zone";
import { useState } from "react";
import { imageService } from "~/services/images";

type Payload = StoreProductPayload;

export const Create = () => {
  const revalidator = useRevalidator();
  const form = useForm<Payload>({
    resolver: zodResolver(storeProductSchema),
    values: {
      name: "",
      description: "",
      unit_id: "",
      category_id: "",
      images: [],
      variants: [],
    },
  });

  const [isPending, setIsPending] = useState(false);

  const onSubmit = async (payload: Payload) => {
    setIsPending(true);

    try {
      const images = await Promise.all(
        payload.images.map((file) => imageService.upload(file))
      );

      await productService.store({
        ...payload,
        images: images.map(({ id }) => id),
        variants: payload.variants.filter((variant) => variant.checked),
      });

      revalidator.revalidate();
      toast.success("Product added successfulyy");
    } catch (err) {
      console.error(err);
      toast.error("Something went wrong");
    }

    setIsPending(false);
  };

  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button>New product</Button>
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6 overflow-y-auto">
        <SheetHeader>
          <SheetTitle>New Product</SheetTitle>
          <SheetDescription>
            You can new products using the form down below.
          </SheetDescription>
        </SheetHeader>

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
                  <FormDescription>
                    The category of the product.
                  </FormDescription>
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
                  <FormDescription>
                    The description of the product.
                  </FormDescription>
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
                    <DropZone
                      onValueChange={field.onChange}
                      value={field.value}
                    />
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
                    <Variants
                      onValueChange={field.onChange}
                      value={field.value}
                    />
                  </FormControl>
                  <FormDescription>
                    The variants of the product.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button className="w-fit" type="submit" isPending={isPending}>
              Create
            </Button>
          </form>
        </Form>
      </SheetContent>
    </Sheet>
  );
};
