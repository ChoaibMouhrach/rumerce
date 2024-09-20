import { useRevalidator } from "@remix-run/react";
import { useMutation } from "@tanstack/react-query";
import React from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { productService } from "@/services/products";

interface DeleteProps {
  id: string;
  close: () => void;
}

export const Delete: React.FC<DeleteProps> = ({ id, close }) => {
  const revalidator = useRevalidator();

  const { mutate, isPending } = useMutation({
    mutationFn: productService.destroy,
    onSuccess: () => {
      toast.success("Product deleted successfully");
      revalidator.revalidate();
      close();
    },
    onError: () => {
      toast.error("Something went wrong");
    },
  });

  const onDelete = () => {
    mutate(id);
  };

  return (
    <Button isPending={isPending} onClick={onDelete} variant="destructive">
      Delete
    </Button>
  );
};
