export type BookRequest = {
  name: string;
  description?: string;
  author: string;
  year: number;
};

export type Book = {
  _id: string;
  name: string;
  description?: string;
  author: string;
  year: number;
  media?: {
    id: string;
    ext: string;
  };
};
