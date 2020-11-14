
const alpha = c => c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
function words(s: string): String[] {
  let a = s.split()
  return a.map(c => alpha(c) ? c : ' ').join("").split(/ /g)
}

const log = n => console.log(JSON.stringify(n))

log(words("Hello World! My name is Nils."))
