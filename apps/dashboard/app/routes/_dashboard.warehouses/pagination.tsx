import { Button } from "@/components/ui/button";

export const Pagination = () => {
  return (
    <div className="flex items-center justify-between">
      <span>Page 1 of 100</span>
      <div className="flex items-center gap-2">
        <Button size="sm" variant="secondary">
          Previous
        </Button>
        <Button size="sm" variant="secondary">
          Next
        </Button>
      </div>
    </div>
  );
};
