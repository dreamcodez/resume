use crate::models::Job;

pub fn get_jobs() -> Vec<Job> {
    vec![
        Job {
            company: "Umee (Now UX Chain)".to_string(),
            company_description: "UX Chain, formerly UMEE, is a layer-one blockchain within the Cosmos ecosystem, dedicated to enhancing user experience and fostering innovation in decentralised finance (DeFi). As a universal cross-chain DeFi hub, it provides the groundwork for future DApps and financial components, known as \"money legos,\" within the DeFi landscape.".to_string(),
            company_website: "https://www.ux.xyz".to_string(),
            job_title: "Head Infrastructure Engineer".to_string(),
            start: "2021-12".to_string(),
            end: Some("2023-03".to_string()),
            markdown: r#"
* **Led a team of 2 engineers** in provisioning and managing Cosmos blockchain test networks
* **Built custom automation tools** using Pulumi in Golang for genesis ceremony and wallet funding processes
* **Architected and deployed** Google Cloud Platform (GCP) infrastructure with comprehensive monitoring systems
* **Developed and maintained** various Golang codebases for blockchain operations
* **Built out the DevOps team** and implemented new processes for improved operational efficiency
* **Automated deployment** of several blockchain test networks with custom-built tools
* **Recognized for encyclopedic technical knowledge and ability to architect complex systems**
* **Praised for calm leadership under pressure and cross-team collaboration**
            "#.trim().to_string(),
        },
        Job {
            company: "HelloTech Inc.".to_string(),
            company_description: "HelloTech is an innovative b2b application which enables service workers to be dispatched for technical installations such as TV mounts and Wi-Fi installs with partners like Wal-Mart, Amazon & SimpliSafe.".to_string(),
            company_website: "https://www.hellotech.com".to_string(),
            job_title: "Senior Software Engineer".to_string(),
            start: "2020-12".to_string(),
            end: Some("2021-12".to_string()),
            markdown: r#"
* **Architected core systems** using Golang and Node.js for high-performance microservices
* **Designed and implemented** new database schemas and protobuf schemas to support feature development
* **Refactored existing databases** to significantly improve performance and scalability
* **Built and maintained** several microservices supporting the dispatch and service worker platform
* **Collaborated on system architecture** decisions and technical roadmap planning
* **Recognized for fast, comprehensive, and adaptive problem solving**
            "#.trim().to_string(),
        },
        Job {
            company: "Endpoint Escrow Inc.".to_string(),
            company_description: "Endpoint is a company which is disrupting escrow/mortgage by streamlining and automating much of the process.".to_string(),
            company_website: "https://endpointclosing.com".to_string(),
            job_title: "Senior Software Engineer".to_string(),
            start: "2019-09".to_string(),
            end: Some("2020-12".to_string()),
            markdown: r#"
* **Implemented production-ready strategy** for communicating with legacy database systems while building new features
* **Facilitated transition** from monolithic architecture to microservice architecture
* **Collaborated on GraphQL API** development to service internal admin needs and improve data access
* **Extended and created** new DynamoDB schemas for emerging services and features
* **Built an effortless closing experience** for all parties in real estate transactions
* **Architected database solutions** that bridge legacy and modern systems seamlessly
* **Praised for ability to integrate R&D findings into legacy products**
            "#.trim().to_string(),
        },
        Job {
            company: "Whiteblock Inc.".to_string(),
            company_description: "Whiteblock sells professional testing services to blockchain technology companies.".to_string(),
            company_website: "https://whiteblock.io".to_string(),
            job_title: "Platform Architect".to_string(),
            start: "2018-12".to_string(),
            end: Some("2019-08".to_string()),
            markdown: r#"
* **Built comprehensive CI/CD frameworks** for blockchain testing and validation
* **Developed data validation frameworks** to test blockchain performance and reliability
* **Architected testing platforms** that help development teams validate performance and deploy distributed ledger technologies faster
* **Created automated testing solutions** for blockchain networks and smart contracts
* **Designed scalable testing infrastructure** for multiple blockchain protocols
* **Endorsed by colleagues for technical depth and leadership**
            "#.trim().to_string(),
        },
        Job {
            company: "BroadVoice Inc.".to_string(),
            company_description: "Broadvoice sells Cloud Voice-Over-IP and Communications services to small, medium, and enterprise businesses.".to_string(),
            company_website: "https://www.broadvoice.com".to_string(),
            job_title: "Software Architect".to_string(),
            start: "2018-05".to_string(),
            end: Some("2018-12".to_string()),
            markdown: r#"
* **Rewritten critical components** of the SIP stack using Rust for improved performance and reliability
* **Implemented message queue systems** to enhance communication infrastructure
* **Architected VoIP solutions** for Small/Medium/Large business customers
* **Designed scalable communication systems** supporting enterprise-level deployments
* **Optimized network protocols** for better call quality and system reliability
* **Recognized for ability to deliver under pressure and across teams**
            "#.trim().to_string(),
        },
        Job {
            company: "LeaseLock Inc.".to_string(),
            company_description: "LeaseLock powers a modern lease experience that totally eliminates all security deposits, surety bonds and guarantors.".to_string(),
            company_website: "https://leaselock.com".to_string(),
            job_title: "Software Architect".to_string(),
            start: "2017-11".to_string(),
            end: Some("2018-04".to_string()),
            markdown: r#"
* **Implemented payment processing system** using Braintree to handle customer transactions securely
* **Built continuous deployment pipeline** using Docker, Kubernetes, and AWS CodeBuild
* **Refined development processes** with Git for faster release cycles and better version control
* **Implemented Apache NiFi** for dataflow integration and real-time data processing
* **Architected scalable payment infrastructure** supporting thousands of transactions
* **Designed automated deployment systems** for improved operational efficiency
* **Endorsed by colleagues for technical expertise and reliability**
            "#.trim().to_string(),
        },
        Job {
            company: "Stem Disintermedia Inc.".to_string(),
            company_description: "Stem helps music collaborators receive royalties in a fair way.".to_string(),
            company_website: "https://stem.is".to_string(),
            job_title: "Principal Platform Architect".to_string(),
            start: "2015-10".to_string(),
            end: Some("2017-07".to_string()),
            markdown: r#"
* **Implemented PostgreSQL** database systems and continuous deployment pipelines
* **Integrated various SaaS platforms** to streamline music royalty distribution
* **Re-architected server deployments** for increased automation and scalability
* **Took lead role** in re-architecting systems for growth, programmer productivity, and robustness
* **Designed distributed systems** for handling complex royalty calculations and payments
* **Built automated deployment systems** for improved development velocity
* **Recognized for leadership and mentoring in technical teams**
            "#.trim().to_string(),
        },
        Job {
            company: "Lambda Software, Inc.".to_string(),
            company_description: "Lambda Software was formed as a way to seek funding for startup ventures and provide software architecture consulting services.".to_string(),
            company_website: "".to_string(),
            job_title: "President".to_string(),
            start: "2014-10".to_string(),
            end: Some("2015-09".to_string()),
            markdown: r#"
* **Founded and led** software consulting company focused on scaling software architectures
* **Provided technical consulting** to clients on system design and scalability challenges
* **Sought funding** for startup ventures and technology innovations
* **Demonstrated entrepreneurial leadership** in software industry
            "#.trim().to_string(),
        },
        Job {
            company: "Standard Crypto".to_string(),
            company_description: "Standard Crypto developed backend technology for processing bitcoin transactions for online gaming platforms.".to_string(),
            company_website: "".to_string(),
            job_title: "Lead Software Developer".to_string(),
            start: "2013-11".to_string(),
            end: Some("2014-09".to_string()),
            markdown: r#"
* **Developed backend technology stack** to process bitcoin transactions for online casino using Node.js/Insight
* **Tuned layout/platform/architecture** of mixxtopia.com to optimize AdSense revenue
* **Built cryptocurrency payment processing** systems for high-volume transactions
* **Optimized web platform performance** and monetization strategies
            "#.trim().to_string(),
        },
        Job {
            company: "Dimension Software".to_string(),
            company_description: "Dimension Software developed modern bulletin-board web applications as a software-as-a-service platform.".to_string(),
            company_website: "https://powerbulletin.com".to_string(),
            job_title: "Co-Founder / Software Developer".to_string(),
            start: "2013-01".to_string(),
            end: Some("2013-10".to_string()),
            markdown: r#"
* **Co-founded software company** developing modern bulletin-board web applications
* **Built from-scratch platform** using Node.js and LiveScript for business SaaS
* **Developed powerbulletin.com** platform currently in beta
* **Demonstrated full-stack development** and entrepreneurial skills
            "#.trim().to_string(),
        },
        Job {
            company: "Weedmaps Media".to_string(),
            company_description: "WeedMaps helps medical marijuana patients find dispensaries and provides helpful reviews and menus.".to_string(),
            company_website: "https://weedmaps.com".to_string(),
            job_title: "Lead Software Developer".to_string(),
            start: "2011-01".to_string(),
            end: Some("2012-12".to_string()),
            markdown: r#"
* **Overhauled search engine** using Elasticsearch with custom weighting algorithms over nearly a year
* **Led development team** of 3 people in building and maintaining various platform features
* **Worked on multiple projects** including coupons, menus, and server optimization (Linux)
* **Implemented server optimizations** for improved performance and reliability
* **Designed and built** features for dispensary discovery and patient services
* **Maintained and scaled** platform supporting thousands of dispensaries and patients
* **Praised for encyclopedic technical knowledge and ability to deliver results**
            "#.trim().to_string(),
        },
        Job {
            company: "Bolthouse Farms".to_string(),
            company_description: "Bolthouse Farms is a leading producer of premium beverages, carrots and super-premium refrigerated dressings.".to_string(),
            company_website: "https://bolthouse.com".to_string(),
            job_title: "Senior Software Developer".to_string(),
            start: "2009-01".to_string(),
            end: Some("2010-12".to_string()),
            markdown: r#"
* **Developed and maintained** enterprise resource planning (ERP) systems
* **Built business intelligence** solutions for agricultural operations
* **Implemented data integration** systems for supply chain management
* **Designed reporting systems** for executive decision making
* **Optimized database performance** for large-scale agricultural data
            "#.trim().to_string(),
        },
    ]
}

pub fn get_job_by_company(company: &str) -> Option<Job> {
    get_jobs().into_iter().find(|job| job.company == company)
}
