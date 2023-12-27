import type { Metadata } from "next";
import { Space_Mono as Font } from "next/font/google";
import "./globals.css";
import MainSidebar from "./_components/sidebar/main-sidebar";
import MainNavbar from "./_components/navbar/main-navbar";

const font = Font({
  weight: ["400", "700"],
  subsets: ["latin"],
  preload: true,
});

export const metadata: Metadata = {
  title: "Starchart",
  description: "Starcharting Application",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={font.className}>
        <MainNavbar />
        <MainSidebar>{children}</MainSidebar>
      </body>
    </html>
  );
}
