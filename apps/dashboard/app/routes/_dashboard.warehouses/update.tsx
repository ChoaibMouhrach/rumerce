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
import { warehouseService } from "@/services/warehouse";
import {
  StoreWarehousePayload,
  storeWarehouseSchema,
  Warehouse,
} from "@/validations/warehouse";

type Payload = StoreWarehousePayload;

interface UpdateProps {
  warehouse: Warehouse;
}

export const Update: React.FC<UpdateProps> = ({ warehouse }) => {
  const revalidator = useRevalidator();

  const form = useForm<Payload>({
    resolver: zodResolver(storeWarehouseSchema),
    values: {
      name: warehouse.name,
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => {
      return warehouseService.update(warehouse.id, payload);
    },
    onSuccess: () => {
      toast.success("Warehouse updated successfully");
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
                <Input {...field} placeholder="Warehouse 1" />
              </FormControl>
              <FormDescription>The name of the warehouse</FormDescription>
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
