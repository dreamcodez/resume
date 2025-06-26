export interface Job {
  company: string;
  companyDescription: string;
  companyWebsite: string;
  jobTitle: string;
  start: string;
  end: string | null;
  markdown: string;
}

export default [
  {
    company: "Umee (Now UX Chain)",
    companyDescription:
      'UX Chain, formerly UMEE, is a layer-one blockchain within the Cosmos ecosystem, dedicated to enhancing user experience and fostering innovation in decentralised finance (DeFi). As a universal cross-chain DeFi hub, it provides the groundwork for future DApps and financial components, known as "money legos," within the DeFi landscape.',
    companyWebsite: "https://www.ux.xyz",
    jobTitle: "Head Infrastructure Engineer",
    start: "2021-12",
    end: "2023-03",
    markdown: `
* **Led a team of 2 engineers** in provisioning and managing Cosmos blockchain test networks
* **Built custom automation tools** using Pulumi in Golang for genesis ceremony and wallet funding processes
* **Architected and deployed** Google Cloud Platform (GCP) infrastructure with comprehensive monitoring systems
* **Developed and maintained** various Golang codebases for blockchain operations
* **Built out the DevOps team** and implemented new processes for improved operational efficiency
* **Automated deployment** of several blockchain test networks with custom-built tools
* **Recognized for encyclopedic technical knowledge and ability to architect complex systems**
* **Praised for calm leadership under pressure and cross-team collaboration**
`,
  },
  {
    company: "HelloTech Inc.",
    companyDescription:
      "HelloTech is an innovative b2b application which enables service workers to be dispatched for technical installations such as TV mounts and Wi-Fi installs with partners like Wal-Mart, Amazon & SimpliSafe.",
    companyWebsite: "https://www.hellotech.com",
    jobTitle: "Senior Software Engineer",
    start: "2020-12",
    end: "2021-12",
    markdown: `
* **Architected core systems** using Golang and Node.js for high-performance microservices
* **Designed and implemented** new database schemas and protobuf schemas to support feature development
* **Refactored existing databases** to significantly improve performance and scalability
* **Built and maintained** several microservices supporting the dispatch and service worker platform
* **Collaborated on system architecture** decisions and technical roadmap planning
* **Recognized for fast, comprehensive, and adaptive problem solving**
`,
  },
  {
    company: "Endpoint Escrow Inc.",
    companyDescription:
      "Endpoint is a company which is disrupting escrow/mortgage by streamlining and automating much of the process.",
    companyWebsite: "https://endpointclosing.com",
    jobTitle: "Senior Software Engineer",
    start: "2019-09",
    end: "2020-12",
    markdown: `
* **Implemented production-ready strategy** for communicating with legacy database systems while building new features
* **Facilitated transition** from monolithic architecture to microservice architecture
* **Collaborated on GraphQL API** development to service internal admin needs and improve data access
* **Extended and created** new DynamoDB schemas for emerging services and features
* **Built an effortless closing experience** for all parties in real estate transactions
* **Architected database solutions** that bridge legacy and modern systems seamlessly
* **Praised for ability to integrate R&D findings into legacy products**
`,
  },
  {
    company: "Whiteblock Inc.",
    companyDescription:
      "Whiteblock sells professional testing services to blockchain technology companies.",
    companyWebsite: "https://whiteblock.io",
    jobTitle: "Platform Architect",
    start: "2018-12",
    end: "2019-08",
    markdown: `
* **Built comprehensive CI/CD frameworks** for blockchain testing and validation
* **Developed data validation frameworks** to test blockchain performance and reliability
* **Architected testing platforms** that help development teams validate performance and deploy distributed ledger technologies faster
* **Created automated testing solutions** for blockchain networks and smart contracts
* **Designed scalable testing infrastructure** for multiple blockchain protocols
* **Endorsed by colleagues for technical depth and leadership**
`,
  },
  {
    company: "BroadVoice Inc.",
    companyDescription:
      "Broadvoice sells Cloud Voice-Over-IP and Communications services to small, medium, and enterprise businesses.",
    companyWebsite: "https://www.broadvoice.com",
    jobTitle: "Software Architect",
    start: "2018-05",
    end: "2018-12",
    markdown: `
* **Rewritten critical components** of the SIP stack using Rust for improved performance and reliability
* **Implemented message queue systems** to enhance communication infrastructure
* **Architected VoIP solutions** for Small/Medium/Large business customers
* **Designed scalable communication systems** supporting enterprise-level deployments
* **Optimized network protocols** for better call quality and system reliability
* **Recognized for ability to deliver under pressure and across teams**
`,
  },
  {
    company: "LeaseLock Inc.",
    companyDescription:
      "LeaseLock powers a modern lease experience that totally eliminates all security deposits, surety bonds and guarantors.",
    companyWebsite: "https://leaselock.com",
    jobTitle: "Software Architect",
    start: "2017-11",
    end: "2018-04",
    markdown: `
* **Implemented payment processing system** using Braintree to handle customer transactions securely
* **Built continuous deployment pipeline** using Docker, Kubernetes, and AWS CodeBuild
* **Refined development processes** with Git for faster release cycles and better version control
* **Implemented Apache NiFi** for dataflow integration and real-time data processing
* **Architected scalable payment infrastructure** supporting thousands of transactions
* **Designed automated deployment systems** for improved operational efficiency
* **Endorsed by colleagues for technical expertise and reliability**
`,
  },
  {
    company: "Stem Disintermedia Inc.",
    companyDescription:
      "Stem helps music collaborators receive royalties in a fair way.",
    companyWebsite: "https://stem.is",
    jobTitle: "Principal Platform Architect",
    start: "2015-10",
    end: "2017-07",
    markdown: `
* **Implemented PostgreSQL** database systems and continuous deployment pipelines
* **Integrated various SaaS platforms** to streamline music royalty distribution
* **Re-architected server deployments** for increased automation and scalability
* **Took lead role** in re-architecting systems for growth, programmer productivity, and robustness
* **Designed distributed systems** for handling complex royalty calculations and payments
* **Built automated deployment systems** for improved development velocity
* **Recognized for leadership and mentoring in technical teams**
`,
  },
  {
    company: "Weedmaps Media",
    companyDescription:
      "WeedMaps helps medical marijuana patients find dispensaries and provides helpful reviews and menus.",
    companyWebsite: "https://weedmaps.com",
    jobTitle: "Lead Software Developer",
    start: "2011-01",
    end: "2012-12",
    markdown: `
* **Overhauled search engine** using Elasticsearch with custom weighting algorithms over nearly a year
* **Led development team** of 3 people in building and maintaining various platform features
* **Worked on multiple projects** including coupons, menus, and server optimization (Linux)
* **Implemented server optimizations** for improved performance and reliability
* **Designed and built** features for dispensary discovery and patient services
* **Maintained and scaled** platform supporting thousands of dispensaries and patients
* **Praised for encyclopedic technical knowledge and ability to deliver results**
`,
  },
] as Job[];
