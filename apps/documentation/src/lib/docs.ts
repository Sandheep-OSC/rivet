import type { Doc } from '$lib/utils/types';

// Placeholder documentation data - will be populated when MDX files are added
export const docs: Doc[] = [
  {
    slug: 'intro',
    title: 'Introduction',
    description: 'Learn about Rivet and how to get started',
    section: 'Getting Started',
    order: 1,
  },
  {
    slug: 'api',
    title: 'API Reference',
    description: 'Complete API documentation for Rivet',
    section: 'Reference',
    order: 2,
  },
  {
    slug: 'guides',
    title: 'Guides',
    description: 'Step-by-step tutorials and guides',
    section: 'Tutorials',
    order: 3,
  },
];

// Helper functions for navigation
export function getDocBySlug(slug: string): Doc | undefined {
  return docs.find((doc) => doc.slug === slug);
}

export function getNextDoc(currentSlug: string): string | null {
  const currentIndex = docs.findIndex((doc) => doc.slug === currentSlug);
  if (currentIndex === -1 || currentIndex === docs.length - 1) return null;
  return docs[currentIndex + 1].slug;
}

export function getPreviousDoc(currentSlug: string): string | null {
  const currentIndex = docs.findIndex((doc) => doc.slug === currentSlug);
  if (currentIndex <= 0) return null;
  return docs[currentIndex - 1].slug;
}
