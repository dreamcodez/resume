export interface Skill {
  name: string;
  level: "Expert" | "Fluent" | "Amateur" | "Basics";
}

export default [
  // Programming Languages
  { name: "JavaScript/Node.js", level: "Expert" },
  { name: "TypeScript", level: "Expert" },
  { name: "Golang", level: "Expert" },
  { name: "Rust", level: "Fluent" },
  { name: "Python", level: "Fluent" },
  { name: "Ruby/Ruby on Rails", level: "Expert" },
  { name: "Java", level: "Fluent" },
  { name: "C/C++", level: "Fluent" },
  { name: "Haskell", level: "Fluent" },
  { name: "BASH", level: "Fluent" },
  { name: "Perl", level: "Amateur" },
  { name: ".NET", level: "Amateur" },
  { name: "Erlang", level: "Basics" },
  { name: "Smalltalk", level: "Basics" },
  { name: "Visual Works", level: "Basics" },
  { name: "AmberJS", level: "Basics" },
  { name: "LiveScript", level: "Fluent" },

  // Web Technologies
  { name: "React", level: "Expert" },
  { name: "GraphQL", level: "Expert" },
  { name: "REST APIs", level: "Expert" },
  { name: "Webpack", level: "Expert" },
  { name: "Babel", level: "Expert" },
  { name: "MobX", level: "Fluent" },
  { name: "Redux", level: "Fluent" },
  { name: "Relay", level: "Fluent" },

  // Databases & Data
  { name: "PostgreSQL", level: "Expert" },
  { name: "MySQL", level: "Expert" },
  { name: "SQL", level: "Expert" },
  { name: "DynamoDB", level: "Fluent" },
  { name: "Graph Databases", level: "Fluent" },
  { name: "Graph Data Structures", level: "Amateur" },
  { name: "BigQuery", level: "Fluent" },
  { name: "Apache NiFi", level: "Fluent" },

  // Cloud & Infrastructure
  { name: "Kubernetes", level: "Expert" },
  { name: "Docker", level: "Expert" },
  { name: "AWS", level: "Expert" },
  { name: "Google Cloud Platform", level: "Expert" },
  { name: "Terraform", level: "Fluent" },
  { name: "Packer", level: "Fluent" },
  { name: "Pulumi", level: "Fluent" },
  { name: "CloudFormation", level: "Fluent" },
  { name: "kops", level: "Fluent" },
  { name: "AWS CodePipeline", level: "Fluent" },
  { name: "AWS CodeBuild", level: "Fluent" },
  { name: "Google Cloud Build", level: "Fluent" },
  { name: "Google PubSub", level: "Fluent" },

  // DevOps & CI/CD
  { name: "Git", level: "Expert" },
  { name: "Mercurial", level: "Fluent" },
  { name: "Version Control", level: "Expert" },
  { name: "Continuous Deployment", level: "Expert" },
  { name: "CI/CD", level: "Expert" },
  { name: "Build Pipelines", level: "Expert" },

  // Blockchain & Cryptocurrency
  { name: "Blockchain", level: "Expert" },
  { name: "Cosmos", level: "Expert" },
  { name: "Bitcoin", level: "Fluent" },
  { name: "Cryptocurrency", level: "Fluent" },
  { name: "Distributed Systems", level: "Expert" },
  { name: "Genesis Ceremony", level: "Expert" },

  // System Administration
  { name: "Linux", level: "Expert" },
  { name: "Mac OS X", level: "Expert" },
  { name: "BSD", level: "Expert" },
  { name: "Windows", level: "Fluent" },
  { name: "VMware", level: "Fluent" },
  { name: "System Administration", level: "Expert" },
  { name: "DNS", level: "Fluent" },
  { name: "Email Systems", level: "Fluent" },
  { name: "Intrusion Detection", level: "Fluent" },

  // Business Intelligence & Analytics
  { name: "Business Objects BI", level: "Fluent" },
  { name: "Data Analytics", level: "Fluent" },
  { name: "SEO", level: "Fluent" },
  { name: "Performance Optimization", level: "Expert" },

  // Communication & Protocols
  { name: "SIP", level: "Fluent" },
  { name: "VoIP", level: "Fluent" },
  { name: "Network Protocols", level: "Fluent" },
  { name: "Message Queues", level: "Fluent" },
  { name: "RabbitMQ", level: "Fluent" },

  // Architecture & Design
  { name: "Microservices", level: "Expert" },
  { name: "System Architecture", level: "Expert" },
  { name: "API Design", level: "Expert" },
  { name: "Database Design", level: "Expert" },
  { name: "Scalability", level: "Expert" },
  { name: "Performance Tuning", level: "Expert" },
  { name: "Infrastructure as Code", level: "Expert" },

  // Payment & Financial
  { name: "Payment Processing", level: "Fluent" },
  { name: "Braintree", level: "Fluent" },
  { name: "Financial Systems", level: "Fluent" },
  { name: "Transaction Processing", level: "Fluent" },

  // Testing & Quality
  { name: "Testing Frameworks", level: "Fluent" },
  { name: "Blockchain Testing", level: "Expert" },
  { name: "Performance Testing", level: "Fluent" },
  { name: "Automated Testing", level: "Expert" },

  // Leadership & Soft Skills
  { name: "Technical Leadership", level: "Expert" },
  { name: "Team Management", level: "Expert" },
  { name: "Mentoring", level: "Expert" },
  { name: "Cross-functional Collaboration", level: "Expert" },
  { name: "Problem Solving", level: "Expert" },
  { name: "Architecture Planning", level: "Expert" },
  { name: "Project Management", level: "Fluent" },
  { name: "Customer Satisfaction", level: "Expert" },
  { name: "Entrepreneurship", level: "Fluent" },
  { name: "Consulting", level: "Expert" },

  // Languages
  { name: "English", level: "Expert" },
  { name: "Spanish", level: "Basics" },
] as Skill[];
