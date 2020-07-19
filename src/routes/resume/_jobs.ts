export interface Job {
    company: string;
    companyDescription: string;
    jobTitle: string;
    start: string;
    end: string | null;
    markdown: string;
}

export default [
    {
        company: 'Endpoint Escrow Inc.',
        companyDescription: 'Endpoint is a company which is disrupting escrow/mortgage by streamlining and automating much of the process.',
        jobTitle: 'Senior Software Engineer',
        start: '2019-08',
        end: null,
        markdown: `
            * Implemented (in production) a strategy to communicate with the existing legacy database while building new features.
            * Helping facilitate transition to Microservice Architecture from our monolith
            * Collaborated on GraphQL api to service our internal admin needs
            * Extended & Created new databases schemas (dynamo) for new services being built
        `,
    },
    {
        company: 'Whiteblock Inc.',
        companyDescription: 'Whiteblock sells professional testing services to blockchain technology companies.',
        jobTitle: 'Platform Architect',
        start: '2018-12',
        end: '2019-08',
        markdown: `
        `,
    },
    {
        company: 'BroadVoice Inc.',
        companyDescription: 'Broadvoice sells Cloud Voice-Over-IP and Communications services to small, medium, and enterprise businesses.',
        jobTitle: 'Software Architect',
        start: '2018-05',
        end: '2018-12',
        markdown: `
        `,
    },
    {
        company: 'LeaseLock Inc.',
        companyDescription: 'TODO',
        jobTitle: 'Software Architect',
        start: '2017-11',
        end: '2018-04',
        markdown: `
        `,
    },
    {
        company: 'Stem Disintermedia Inc.',
        companyDescription: 'Stem helps music collaborators receive royalties in a fair way.',
        jobTitle: 'Principal Platform Architect',
        start: '2015-10',
        end: '2017-07',
        markdown: `
        `,
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