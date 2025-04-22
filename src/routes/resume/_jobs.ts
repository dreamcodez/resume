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
      "UX Chain, formerly UMEE, is a layer-one blockchain within the Cosmos ecosystem, dedicated to enhancing user experience and fostering innovation in decentralised finance (DeFi). As a universal cross-chain DeFi hub, it provides the groundwork for future DApps and financial components, known as “money legos,” within the DeFi landscape.",
    companyWebsite: "https://www.ux.xyz",
    jobTitle: "Head Infrastructure Engineer",
    start: "2021-12",
    end: "2023-03",
    markdown: `
Automated deployment of several blockchain test networks. Used Pulumi in golang to automate genesis ceremony and wallet funding on these test networks. Built out & trained a team to manage the devops & infrastructure side.`,
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
Worked on several microservices and designed new database and protobuf schemas to support feature work. Refactored database to improve performance. `,
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
* Implemented (in production) a strategy to communicate with the existing legacy database while building new features.
* Helping facilitate transition to Microservice Architecture from our monolith
* Collaborated on GraphQL api to service our internal admin needs
* Extended & Created new databases schemas (dynamo) for new services being built`,
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
Worked on various projects including coupons, menus, and server optimization (linux).
    
I spent nearly a year overhauling the search engine in elasticsearch with custom weights. Small team of 3 people.`,
  },
] as Job[];

/*
##### Software Engineer, Endpoint Escrow Inc.
###### August 2019 to Present
Endpoint is a company which is disrupting escrow/mortgage by streamlining and automating much of the process.

* Implemented (in production) a strategy to communicate with the existing legacy database while building new features.
* Helping facilitate transition to Microservice Architecture from our monolith
* Collaborated on GraphQL api to service our internal admin needs
* Extended & Created new databases schemas (dynamo) for new services being built
*/
