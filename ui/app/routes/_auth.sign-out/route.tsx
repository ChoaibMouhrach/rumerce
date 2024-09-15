import { LoaderFunctionArgs } from "@remix-run/node";
import { useNavigate } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";
import { Button } from "~/components/ui/button";
import { auth } from "~/services/auth";

export const loader = ({ request }: LoaderFunctionArgs) => {
  return auth.protected(request);
};

const Page = () => {
  const navigate = useNavigate();
  const { mutate, isPending } = useMutation({
    mutationFn: () => auth.signOut(),
    onSuccess: () => {
      toast.success("See you later!");
      navigate("/");
    },
    onError: () => toast.error("Something went wrong"),
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
      <Button variant="outline" isPending={isPending} onClick={onSignOut}>
        Sign Out
      </Button>
    </div>
  );
};

export default Page;
