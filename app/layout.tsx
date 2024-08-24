import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.scss";
import "./tailwind.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Minnal POS",
  description: "A lightning fast open source point-of-sale system",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
