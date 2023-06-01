export enum FontDisplay {
    Auto = 'auto',
    Block = 'block',
    /** default */
    Swap = 'swap',
    Fallback = 'fallback',
    Optional = 'optional',
  }
  export enum AllowLocal {
    /** default */
    Never = 'false',
    Always = 'true',
    /** default on server */
    Auto = 'auto',
  }
  export enum Weights {
    Normal = 'n',
    Bold = 'b',
    BoldItalic = 'bi',
  }
  export type _Weight = number | Weights
  export type Font = string | {
    family: string;
    /** default: 500 */
    weight?: _Weight | _Weight[];
    /** italic */
    italic?: boolean | boolean[];
  };