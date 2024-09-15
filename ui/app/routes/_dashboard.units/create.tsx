import { zodResolver } from "@hookform/resolvers/zod";
import { useRevalidator } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { Button } from "~/components/ui/button";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  Form,
} from "~/components/ui/form";
import { Input } from "~/components/ui/input";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "~/components/ui/sheet";
import { unitService } from "~/services/units";
import { StoreUnitPayload, storeUnitSchema } from "~/validations/unit";

type Payload = StoreUnitPayload;

export const Create = () => {
  const revalidator = useRevalidator();

  const form = useForm<Payload>({
    resolver: zodResolver(storeUnitSchema),
    values: {
      name: "",
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => unitService.store(payload),
    onSuccess: () => {
      toast.success("Unit added successfully");
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
    <Sheet>
      <SheetTrigger asChild>
        <Button size="sm">New Unit</Button>
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6">
        <SheetHeader>
          <SheetTitle>New Unit</SheetTitle>
          <SheetDescription>
            Use the form down below to add new units to your store.
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
                    <Input {...field} placeholder="KG" />
                  </FormControl>
                  <FormDescription>The name of the unit</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button isPending={isPending} type="submit" className="w-fit">
              Add
            </Button>
          </form>
        </Form>
      </SheetContent>
    </Sheet>
  );
};
