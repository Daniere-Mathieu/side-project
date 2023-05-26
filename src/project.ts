export enum Status {
  Finished = "finished", // check icon
  Active = "active", // play icon
  Inactive = "inactive", // pause icon
  Archived = "archived", // archive icon
  Deleted = "deleted", // close icon
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
