import { useLoaderData, useNavigate } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";
import { Button } from "~/components/ui/button";
import { env } from "~/env";

export const loader = () => ({
  ENV: {
    API_URL: env.API_URL,
  },
});

const Page = () => {
  const { ENV } = useLoaderData<typeof loader>();
  const navigate = useNavigate();
  const { mutate, isPending } = useMutation({
    mutationFn: async () => {
      const url = new URL(ENV.API_URL);
      url.pathname = "/sign-out";

      await fetch(url, {
        credentials: "include",
        method: "POST",
      });
    },
    onSuccess: () => {
      toast.success("See you soon");
      navigate("/");
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const onBack = () => {
    navigate(-1);
  };

  const onSignOut = () => {
    mutate();
  };

  return (
    <div className="grid grid-cols-2 gap-2">
      <Button disabled={isPending} onClick={onBack}>
        Back
      </Button>
      <Button isPending={isPending} onClick={onSignOut} variant="secondary">
        Sign Out
      </Button>
    </div>
  );
};

export default Page;
