import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { env } from "@/env";
import { api } from "@/services/auth";
import { zodResolver } from "@hookform/resolvers/zod";
import { useNavigate } from "@remix-run/react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

const schema = z.object({
  email: z.string().email(),
});

type Payload = z.infer<typeof schema>;

const Setup = () => {
  const navigate = useNavigate();
  const form = useForm<Payload>({
    resolver: zodResolver(schema),
    values: {
      email: "",
    },
  });

  const [isPending, setIsPending] = useState(false);

  const onSubmit = async (payload: Payload) => {
    setIsPending(true);

    const url = new URL(env.VITE_API_URL);
    url.pathname = "/setup";

    try {
      const response = await api(url, {
        method: "POST",
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(response.statusText);
      }

      form.reset();
      navigate("/");
    } catch (err) {
      console.error(err);
      toast.error("Something went wrong");
    }

    setIsPending(false);
  };

  return (
    <main className="flex items-center justify-center min-h-[100dvh]">
      <div className="max-w-md w-full flex flex-col gap-6">
        <div className="flex flex-col gap-1">
          <h1 className="font-bold text-4xl">Setup</h1>
          <span className="text-muted-foreground">
            You can setup your e-commerce application from here.
          </span>
        </div>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex flex-col gap-2"
          >
            <FormField
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Admin Email address</FormLabel>
                  <FormControl>
                    <Input {...field} placeholder="example@example.com" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button isPending={isPending} type="submit">
              Submit
            </Button>
          </form>
        </Form>
      </div>
    </main>
  );
};

export default Setup;
