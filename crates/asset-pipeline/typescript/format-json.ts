export const formatter = () => {
    document.querySelectorAll('pre.json').forEach((pre) => {
        var json =JSON.parse(pre.innerHTML)
        var formatted = JSON.stringify(json, null, "\t");
        pre.innerHTML = formatted
    })
}
