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
        `
    }

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