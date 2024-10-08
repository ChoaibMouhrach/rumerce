import { useRevalidator } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import React from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { userService } from "@/services/user";

interface DeleteProps {
  close: () => void;
  id: string;
}

export const Delete: React.FC<DeleteProps> = ({ close, id }) => {
  const revalidator = useRevalidator();

  const { mutate, isPending } = useMutation({
    mutationFn: () => userService.destroy(id),
    onSuccess: () => {
      toast.success("User deleted successfully");
      revalidator.revalidate();
      close();
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const onDelete = () => {
    mutate();
  };

  return (
    <Button isPending={isPending} variant="destructive" onClick={onDelete}>
      Delete
    </Button>
  );
};
