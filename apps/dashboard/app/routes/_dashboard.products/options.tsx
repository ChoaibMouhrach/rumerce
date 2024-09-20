import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import React, { useState } from "react";
import { productService } from "@/services/products";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Delete } from "./delete";
import { Update } from "./update";

interface OptionsProps {
  product: Awaited<ReturnType<typeof productService.all>>[number];
  children: React.ReactNode;
}

export const Options: React.FC<OptionsProps> = ({ children, product }) => {
  const [open, setOpen] = useState(false);

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild className="cursor-pointer">
        {children}
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6 overflow-y-auto">
        <SheetHeader>
          <SheetTitle>Are you absolutely sure?</SheetTitle>
          <SheetDescription>
            This action cannot be undone. This will permanently delete your
            account and remove your data from our servers.
          </SheetDescription>
        </SheetHeader>

        <Tabs defaultValue="update">
          <TabsList>
            <TabsTrigger value="update">Update</TabsTrigger>
            <TabsTrigger value="delete">Delete</TabsTrigger>
          </TabsList>
          <TabsContent value="update">
            <Update product={product} />
          </TabsContent>
          <TabsContent value="delete">
            <Delete close={() => setOpen(false)} id={product.product.id} />
          </TabsContent>
        </Tabs>
      </SheetContent>
    </Sheet>
  );
};
