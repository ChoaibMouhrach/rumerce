import { zodResolver } from "@hookform/resolvers/zod";
import { useRevalidator } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import React from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  Form,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { categoryService } from "@/services/categories";
import {
  Category,
  StoreCategoryPayload,
  storeCategorySchema,
} from "@/validations/category";

type Payload = StoreCategoryPayload;

interface UpdateProps {
  category: Category;
}

export const Update: React.FC<UpdateProps> = ({ category }) => {
  const revalidator = useRevalidator();

  const form = useForm<Payload>({
    resolver: zodResolver(storeCategorySchema),
    values: {
      name: category.name,
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => {
      return categoryService.update(category.id, payload);
    },
    onSuccess: () => {
      toast.success("Category updated successfully");
      form.reset();
      revalidator.revalidate();
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const onSubmit = (payload: Payload) => {
    mutate(payload);
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
                <Input {...field} placeholder="Category 1" />
              </FormControl>
              <FormDescription>The name of the category</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button isPending={isPending} type="submit" className="w-fit">
          Save
        </Button>
      </form>
    </Form>
  );
};
