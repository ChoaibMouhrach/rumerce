import { Settings } from "lucide-react";
import { Button } from "~/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "~/components/ui/sheet";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs";
import { Update } from "./update";
import { Delete } from "./delete";
import React, { useState } from "react";
import { User } from "~/validations/user";
import { Role } from "~/validations/auth";

interface OptionsProps {
  user: User;
  roles: Role[];
}

export const Options: React.FC<OptionsProps> = ({ user, roles }) => {
  const [open, setOpen] = useState(false);

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild>
        <Button size="sm" variant="ghost">
          <Settings className="w-4 h-4" />
        </Button>
      </SheetTrigger>
      <SheetContent className="flex flex-col gap-6">
        <SheetHeader>
          <SheetTitle>User settings</SheetTitle>
          <SheetDescription>
            You can manage this user from here.
          </SheetDescription>
        </SheetHeader>

        <Tabs defaultValue="update">
          <TabsList>
            <TabsTrigger value="update">Update</TabsTrigger>
            <TabsTrigger value="delete">Delete</TabsTrigger>
          </TabsList>
          <TabsContent value="update">
            <Update user={user} roles={roles} />
          </TabsContent>
          <TabsContent value="delete">
            <Delete id={user.id} close={() => setOpen(false)} />
          </TabsContent>
        </Tabs>
      </SheetContent>
    </Sheet>
  );
};
