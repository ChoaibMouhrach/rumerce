import { useForm } from "react-hook-form";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "~/components/ui/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { useMutation } from "@tanstack/react-query";
import { auth } from "~/services/auth";
import { toast } from "sonner";
import { LoaderFunction, redirect } from "@remix-run/node";

export const loader: LoaderFunction = async ({ request }) => {
  const cookies = request.headers.get("cookie");

  if (!cookies) {
    return {};
  }

  if (!cookies.includes("session=")) {
    return {};
  }

  return redirect("/");
};

const schema = z.object({
  email: z.string().email(),
});

type Payload = z.infer<typeof schema>;

const Page = () => {
  const form = useForm<Payload>({
    resolver: zodResolver(schema),
    values: {
      email: "",
    },
  });

  const { mutate, isPending } = useMutation({
    mutationFn: (payload: Payload) => auth.signIn(payload.email),
    onSuccess: () => {
      toast.success("Check your inbox");
      form.reset();
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
        className="flex flex-col gap-2"
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <FormField
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Input {...field} placeholder="example@example.com" />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button isPending={isPending} type="submit">
          Sign In
        </Button>
      </form>
    </Form>
  );
};

export default Page;
