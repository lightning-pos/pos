import dynamic from 'next/dynamic';

/**
 * Only use Drizzle in client side rendering
 */
export const DynamicDrizzleProvider = dynamic(() => import('./drizzle_provider').then(mod => mod.DrizzleProvider), { ssr: false });
