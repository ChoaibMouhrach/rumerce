import { zodResolver } from "@hookform/resolvers/zod";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { useRevalidator } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import React from "react";
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
import { userService } from "~/services/user";
import { Role } from "~/validations/auth";
import { StoreUserPayload, storeUserSchema } from "~/validations/user";

type Payload = StoreUserPayload;

interface CreateProps {
  roles: Role[];
}

export const Create: React.FC<CreateProps> = ({ roles }) => {
  const revalidator = useRevalidator();

  const form = useForm<Payload>({
    resolver: zodResolver(storeUserSchema),
    values: {
      name: "",
      email: "",
      role_id: "",
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => userService.store(payload),
    onSuccess: () => {
      toast.success("User added successfully");
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
        <Button size="sm">New User</Button>
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6">
        <SheetHeader>
          <SheetTitle>New User</SheetTitle>
          <SheetDescription>
            Use the form down below to add new users to your store.
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
                    <Input {...field} placeholder="John Doe" />
                  </FormControl>
                  <FormDescription>The name of the user</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input {...field} placeholder="example@example.com" />
                  </FormControl>
                  <FormDescription>The name of the user</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              name="role_id"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Role</FormLabel>
                  <FormControl>
                    <Select onValueChange={field.onChange} value={field.value}>
                      <SelectTrigger>
                        <SelectValue placeholder="Role" />
                      </SelectTrigger>
                      <SelectContent>
                        {roles.map((role) => (
                          <SelectItem key={role.id} value={role.id}>
                            {role.name[0].toUpperCase() + role.name.slice(1)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </FormControl>
                  <FormDescription>The role of the user</FormDescription>
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
