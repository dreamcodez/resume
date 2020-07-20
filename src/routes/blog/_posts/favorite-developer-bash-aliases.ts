const markdownBody = `
I was recently reformatting my laptop for the nth time in the past year and I have been carrying around some bash aliases i've used for years. They need to be on my machine because otherwise my muscle memory will not work :)

A bit ago I decided to put it in a pubic git repo to share and for easy reference for myself. It is available here:

https://github.com/dreamcodez/shell-shortcuts/blob/master/.shortcuts

Here is a quick explanation of my favorite aliases:
  * \`\`\`
  alias pbcopy='xclip -selection clipboard'
  alias pbpaste='xclip -selection clipboard -o'
  \`\`\`
  If you've missed the 'pbcopy' and 'pbpaste' commands on MacOS X while developing on Linux -- look no further. I have tested these commands extensively so it should also work for you ;)

  * \`\`\`
  alias gs='git status'
  \`\`\`
  Typically a developer has to type the \`git status\` command very often -- shortened to \`gs\`.

  * \`\`\`
  alias gcp='git checkout -p'
  alias gap='git add -p'
  \`\`\`
  Progressive add and remove commands for the git cache -- these commands are basically the opposite of each other and remind me of the best parts of what [Darcs](https://en.wikipedia.org/wiki/Darcs) was trying to do.
  \`gcp\` will allow you to incrementally and selectively remove new lines of code modification whereas \`gap\` will allow you to incrementally and selectively add new lines of code modification. This allows you to partially check in files and omit parts which you don't want such as a useful \`console.log\` which might only be on your dev environment.

  The other benefit here is that you get to review the code -- one small diff at a time and follow your changes a bit easier. I will typically use this time to also jot down any notes which should go in the commit message as the changes I made are refreshed in my mind.
    
`;

export default {
    title: 'My Favorite Developer BASH aliases',
    slug: 'favorite-developer-bash-aliases',
    date: '2020-07-19',
    markdownBody,
} as Post;