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
import { unitService } from "@/services/units";
import { StoreUnitPayload, storeUnitSchema, Unit } from "@/validations/unit";

type Payload = StoreUnitPayload;

interface UpdateProps {
  unit: Unit;
}

export const Update: React.FC<UpdateProps> = ({ unit }) => {
  const revalidator = useRevalidator();

  const form = useForm<Payload>({
    resolver: zodResolver(storeUnitSchema),
    values: {
      name: unit.name,
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => {
      return unitService.update(unit.id, payload);
    },
    onSuccess: () => {
      toast.success("Unit updated successfully");
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
                <Input {...field} placeholder="Unit 1" />
              </FormControl>
              <FormDescription>The name of the unit</FormDescription>
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
