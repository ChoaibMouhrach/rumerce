import { useQuery } from "@tanstack/react-query";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { categoryService } from "@/services/categories";
import { Skeleton } from "./ui/skeleton";
import React from "react";

interface CategoriesInputProps {
  onValueChange: (value: string) => void;
  value: string;
}

export const CategoriesInput: React.FC<CategoriesInputProps> = ({
  onValueChange,
  value,
}) => {
  const { data, isSuccess } = useQuery({
    queryKey: ["categories"],
    queryFn: () => categoryService.all(),
  });

  if (isSuccess) {
    return (
      <Select value={value} onValueChange={onValueChange}>
        <SelectTrigger>
          <SelectValue placeholder="Select category" />
        </SelectTrigger>
        <SelectContent>
          {data.map((category) => (
            <SelectItem key={category.id} value={category.id}>
              {category.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    );
  }

  return <Skeleton className="h-10" />;
};
