export function html (body: string): string {
  const appName: string = 'cityio'
  const firstPart: string = ` <html>
  <head>
  <title>${appName}</title>
  </head>
  <style>
  body {
    background: #000;
    color: #FFF;
  }
  a {
    color: #DDD;
  }
  </style>
  <body>
  `
  const secondPart: string = '</body></html>'

  return `${firstPart}${body}${secondPart}`
}
