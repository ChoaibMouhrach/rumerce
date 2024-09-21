const Page = () => {
  return (
    <main className="min-h-[100dvh] flex items-center justify-center p-4">
      <div className="text-center flex flex-col gap-2">
        <h1 className="text-4xl font-bold">Permission denied</h1>
        <span className="text-muted-foreground">
          You dont have the correct permission to view this page.
        </span>
      </div>
    </main>
  );
};

export default Page;
