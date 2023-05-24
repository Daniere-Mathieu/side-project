export enum Status {
  Active = "active",
  Inactive = "inactive",
  Archived = "archived",
  Deleted = "deleted",
}

export interface Project {
  id: number;
  name: string;
  description?: string;
  status?: Status;
  created_at: string;
  updated_at: string;
  logo?: string;
}
