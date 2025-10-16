export interface Model {
  name: string;
  url: string;
  description: string;
  details: string[];
  pulls: string;
  tags: string;
  updated: string;
}

export interface ModelVersion {
  version: string;
  context: string;
  size: string;
}

export interface PullProgress {
  status: string;
  digest: string;
  total: number;
  completed: number;
}