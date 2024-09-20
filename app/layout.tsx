import type { Metadata } from "next";
import { IBM_Plex_Sans } from "next/font/google";
import "./globals.scss";
import "./tailwind.css";
import { DynamicSystemProvider } from "@/components/providers/dynamic_system_provider";
import { useEffect } from "react";
import Logger from 'js-logger';

const ibmPlexSans = IBM_Plex_Sans({ weight: "400", subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Minnal POS",
  description: "A lightning fast open source point-of-sale system",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useEffect(() => {
    const initializeDatabases = async () => {
      try {
        Logger.info('Running migrations');
        // await migrate(drizzleDb, { migrationsFolder: './lib/pglite/migrations' });
        Logger.info('Migrations completed');
      } catch (error) {
        Logger.error('Migration failed:', error);
      }
    };

    initializeDatabases();
  }, []);
  return (
    <html lang="en">
      <body className={ibmPlexSans.className}>
        <DynamicSystemProvider>{children}</DynamicSystemProvider>
      </body>
    </html>
  );
}
