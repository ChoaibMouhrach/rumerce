import { useForm } from "react-hook-form";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "~/components/ui/form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";
import { env } from "~/env";
import { useLoaderData } from "@remix-run/react";

export const loader = async () => {
  return {
    ENV: {
      API_URL: env.API_URL,
    },
  };
};

const schema = z.object({
  email: z.string().email(),
});

type Payload = z.infer<typeof schema>;

const Page = () => {
  const data = useLoaderData<typeof loader>();

  const { mutate, isPending } = useMutation({
    mutationFn: async (payload: Payload) => {
      const url = new URL(data.ENV.API_URL);
      url.pathname = "/sign-in";

      await fetch(url, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });
    },
    onSuccess: () => {
      toast.success("Check you inbox");
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const form = useForm<Payload>({
    resolver: zodResolver(schema),
    values: {
      email: "",
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

        <Button type="submit" isPending={isPending}>
          Sign In
        </Button>
      </form>
    </Form>
  );
};

export default Page;
