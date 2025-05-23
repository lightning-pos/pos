import type { Metadata } from 'next'
import { IBM_Plex_Sans } from 'next/font/google'
import './globals.scss'
import './tailwind.css'

const ibmPlexSans = IBM_Plex_Sans({ weight: '400', subsets: ['latin'] })

export const metadata: Metadata = {
    title: 'Minnal POS',
    description: 'A lightning fast open source point-of-sale system',
}

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className={ibmPlexSans.className}>
                {children}
            </body>
        </html >
    )
}
