use crate::models::{Skill, SkillLevel};

pub fn get_skills() -> Vec<Skill> {
    vec![
        // Programming Languages
        Skill {
            name: "JavaScript/Node.js".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "TypeScript".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Golang".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Rust".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Python".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Ruby/Ruby on Rails".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Java".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "C/C++".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Haskell".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "BASH".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Perl".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: ".NET".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Erlang".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Smalltalk".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Visual Works".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "AmberJS".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "LiveScript".to_string(),
            level: SkillLevel::Fluent,
        },
        // Web Technologies
        Skill {
            name: "React".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "GraphQL".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "REST APIs".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Webpack".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Babel".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "MobX".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Redux".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Relay".to_string(),
            level: SkillLevel::Fluent,
        },
        // Databases & Data
        Skill {
            name: "PostgreSQL".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "MySQL".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "SQL".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "DynamoDB".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Graph Databases".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Graph Data Structures".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "BigQuery".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Apache NiFi".to_string(),
            level: SkillLevel::Fluent,
        },
        // Cloud & Infrastructure
        Skill {
            name: "Kubernetes".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Docker".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "AWS".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Google Cloud Platform".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Terraform".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Packer".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Pulumi".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "CloudFormation".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "kops".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "AWS CodePipeline".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "AWS CodeBuild".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Google Cloud Build".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Google PubSub".to_string(),
            level: SkillLevel::Fluent,
        },
        // DevOps & CI/CD
        Skill {
            name: "Git".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Mercurial".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Version Control".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Continuous Deployment".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "CI/CD".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Build Pipelines".to_string(),
            level: SkillLevel::Expert,
        },
        // Blockchain & Cryptocurrency
        Skill {
            name: "Blockchain".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Cosmos".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Bitcoin".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Cryptocurrency".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Distributed Systems".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Genesis Ceremony".to_string(),
            level: SkillLevel::Expert,
        },
        // System Administration
        Skill {
            name: "Linux".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Mac OS X".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "BSD".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Windows".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "VMware".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "System Administration".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "DNS".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Email Systems".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Intrusion Detection".to_string(),
            level: SkillLevel::Fluent,
        },
        // Business Intelligence & Analytics
        Skill {
            name: "Business Objects BI".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Data Analytics".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "SEO".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Performance Optimization".to_string(),
            level: SkillLevel::Expert,
        },
        // Communication & Protocols
        Skill {
            name: "SIP".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "VoIP".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Network Protocols".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Message Queues".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "RabbitMQ".to_string(),
            level: SkillLevel::Fluent,
        },
        // Architecture & Design
        Skill {
            name: "Microservices".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "System Architecture".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "API Design".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Database Design".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Scalability".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Performance Tuning".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Infrastructure as Code".to_string(),
            level: SkillLevel::Expert,
        },
        // Payment & Financial
        Skill {
            name: "Payment Processing".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Braintree".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Financial Systems".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Transaction Processing".to_string(),
            level: SkillLevel::Fluent,
        },
        // Testing & Quality
        Skill {
            name: "Testing Frameworks".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Blockchain Testing".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Performance Testing".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Automated Testing".to_string(),
            level: SkillLevel::Expert,
        },
        // Leadership & Soft Skills
        Skill {
            name: "Technical Leadership".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Team Management".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Mentoring".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Cross-functional Collaboration".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Problem Solving".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Architecture Planning".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Project Management".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Customer Satisfaction".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Entrepreneurship".to_string(),
            level: SkillLevel::Fluent,
        },
        Skill {
            name: "Consulting".to_string(),
            level: SkillLevel::Expert,
        },
        // Languages
        Skill {
            name: "English".to_string(),
            level: SkillLevel::Expert,
        },
        Skill {
            name: "Spanish".to_string(),
            level: SkillLevel::Fluent,
        },
    ]
}

pub fn get_skills_by_level(level: SkillLevel) -> Vec<Skill> {
    get_skills()
        .into_iter()
        .filter(|skill| skill.level == level)
        .collect()
}
