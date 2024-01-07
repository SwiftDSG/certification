export type UserRequest = {
  name: string;
  username: string;
  password: string;
  role: UserRole;
};

export type User = {
  _id: string;
  name: string;
  username: string;
  role: UserRole;
};

export type UserRole = "admin" | "regular";
