export type View = "desktop" | "mobile" | null;

export type InputSwitchOption<T = any> = {
  options:
    | [
        {
          name: string;
          value: T;
        },
        {
          name: string;
          value: T;
        }
      ]
    | [
        {
          name: string;
          value: T;
        },
        {
          name: string;
          value: T;
        },
        {
          name: string;
          value: T;
        }
      ];
  name: string;
  model: {
    name: string;
    value: T;
  };
  disabled?: boolean;
};
export type InputColorOption = {
  color: string;
  alpha: number;
};
export type InputToggleOption = {
  model: boolean;
  disabled?: boolean;
};
export type InputFileOption = {
  file?: File;
  type?: "any" | "image";
  disabled?: boolean;
  label?: string;
  image_url?: string;
  width?: number;
  height?: number;
};
export type InputImageOption = {
  label: string;
  limit: number;
  file: File[];
};
export type InputOption<T = any> = {
  name: string;
  placeholder: string;
  model: {
    name: string;
    value: T;
  };
  prefix?: string;
  icon?: string;
  label?: string;
  error?: string;
  type?: "email" | "password" | "tel" | "number";
  disabled?: boolean;
};
export type InputSelectOption<T = any> = {
  name: string;
  placeholder: string;
  model: {
    name: string;
    value: T;
  };
  prefix?: string;
  icon?: string;
  label?: string;
  error?: string;
  strict?: boolean;
  disabled?: boolean;
  options: {
    name: string;
    value: T;
  }[];
};
export type InputDateOption = {
  name: string;
  model: {
    name: string;
    value: number;
  };
  threshold?: {
    min?: Date;
    max?: Date;
    day?: number[];
  };
  icon?: string;
  label?: string;
  error?: string;
  disabled?: boolean;
};
export type InputTimeOption = {
  model: [number, number];
  label?: string;
};
export type InputCodeOption = {
  name: string;
  length: number;
  model: string;
  disabled?: boolean;
};
export type InputSearchOption = {
  name: string;
  placeholder: string;
  model: string;
};
export type InputGeneric<T> = {
  name: string;
  placeholder: string;
  model: string;
  value?: T;
  icon?: string;
  label?: string;
  error?: string;
  type?: "email" | "password" | "tel" | "number";
  disabled?: boolean;
  options?: (
    | string
    | {
        name: string;
        value: T;
      }
  )[];
};
