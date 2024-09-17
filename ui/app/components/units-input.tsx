import { useQuery } from "@tanstack/react-query";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { unitService } from "~/services/units";
import { Skeleton } from "./ui/skeleton";
import React from "react";

interface UnitsInputProps {
  onValueChange: (value: string) => void;
  value: string;
}

export const UnitsInput: React.FC<UnitsInputProps> = ({
  onValueChange,
  value,
}) => {
  const { data, isSuccess } = useQuery({
    queryKey: ["units"],
    queryFn: () => unitService.all(),
  });

  if (isSuccess) {
    return (
      <Select value={value} onValueChange={onValueChange}>
        <SelectTrigger>
          <SelectValue placeholder="Select unit" />
        </SelectTrigger>
        <SelectContent>
          {data.map((unit) => (
            <SelectItem key={unit.id} value={unit.id}>
              {unit.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    );
  }

  return <Skeleton className="h-10" />;
};
