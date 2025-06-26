export interface Education {
  institution: string;
  degree: string;
  field: string;
  start: string;
  end: string;
  description?: string;
}

export default [
  {
    institution: "California State University",
    degree: "Computer Science",
    field: "Computer Science",
    start: "2005",
    end: "2008",
    description: "Some upper division coursework completed",
  },
] as Education[];
