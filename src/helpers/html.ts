export function html (body: string): string {
  const appName: string = 'cityio'
  const firstPart: string = `
  <html>
  <head>
  <title>${appName}</title>
  <link rel="stylesheet" type="text/css" href="http://yasushisakai.com/static/default.css">
  </head>
  <body>
    `
  const secondPart: string = `</body></html>`

  return `${firstPart}${body}${secondPart}`
}
