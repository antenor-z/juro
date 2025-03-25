document.getElementById("initial").addEventListener("blur", (e) => format(e))
document.getElementById("interest").addEventListener("blur", (e) => format(e))
document.getElementById("time").addEventListener("blur", (e) => format(e, true))
document.getElementById("contribution").addEventListener("blur", (e) => format(e))

function format(e, integer = false) {
    const value_flt = parseFloat(e.target.value.replaceAll(".", "").replace(",", "."))
    if (isNaN(value_flt)) {
        e.target.value = ""
        return
    }
    const digits = integer ? 0 : 2
    const value_formated_str = value_flt.toLocaleString("pt-BR", {maximumFractionDigits: digits, minimumFractionDigits: digits})
    e.target.value = value_formated_str
}