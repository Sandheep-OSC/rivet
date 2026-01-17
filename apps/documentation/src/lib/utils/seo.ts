import type { Doc } from '$lib/types';

export function generateMeta(doc: Doc, baseUrl = 'https://docs.rivet.dev') {
  const title = doc.title;
  const description = doc.description || `Learn about ${doc.title} in the Rivet documentation.`;
  const url = `${baseUrl}/docs/${doc.slug}`;

  return {
    title,
    description,
    canonical: url,
    openGraph: {
      title,
      description,
      url,
      type: 'article',
      siteName: 'Rivet Documentation',
    },
    twitter: {
      card: 'summary',
      title,
      description,
    },
  };
}

export function generateDocTitle(doc: Doc): string {
  return `${doc.title} - Rivet Documentation`;
}
