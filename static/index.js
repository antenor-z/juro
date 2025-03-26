function calc() {
    const initial = parseFloat(document.getElementById("initial").value.replaceAll(".", "").replace(",", "."))
    const interest = parseFloat(document.getElementById("interest").value.replaceAll(".", "").replace(",", ".")) / 100
    const time = parseFloat(document.getElementById("time").value)
    const contribution = parseFloat(document.getElementById("contribution").value.replaceAll(".", "").replace(",", "."))

    const interest_unit = document.querySelector('input[name="interest_unit"]:checked').value;
    const time_unit = document.querySelector('input[name="time_unit"]:checked').value;

    const params = new URLSearchParams(
        {
            initial: initial,
            interest: interest,
            interest_unit: interest_unit,
            time: time,
            time_unit: time_unit,
            contribution: contribution ? contribution : 0.0,
        })

    fetch(`/juro?${params.toString()}`)
        .then(response => response.json())
        .then(data => {
            removeTableChart()
            createGraph(data)

            const resultsTableOld = document.getElementById("results-table")
            if (resultsTableOld !== null) {
                resultsTableOld.remove()
            }
            const resultsTable = document.createElement("table")
            resultsTable.id = "results-table"

            const tHeader = document.createElement("tr")
            const monthH = document.createElement("th")
            monthH.innerText = "Tempo"
            const investedH = document.createElement("th")
            investedH.innerText = "Investido"
            const acumulatedH = document.createElement("th")
            acumulatedH.innerText = "Acumulado"
            const increaseH = document.createElement("th")
            increaseH.innerText = "Aumento"
            const increase_interestH = document.createElement("th")
            increase_interestH.innerText = "Juros"

            tHeader.appendChild(monthH)
            tHeader.appendChild(investedH)
            tHeader.appendChild(acumulatedH)
            tHeader.appendChild(increaseH)
            tHeader.appendChild(increase_interestH)
            resultsTable.appendChild(tHeader)


            data.forEach(element => {
                const row = document.createElement("tr")
                const month = document.createElement("td")
                if (element.month < 12) {
                    month.innerText = element.month + "m"
                }
                else if (element.month % 12 == 0) {
                    month.innerText = `${Math.floor(element.month / 12)}a`
                }
                else {
                    month.innerText = `${Math.floor(element.month / 12)}a ${element.month % 12}m`
                }

                const invested = document.createElement("td")

                invested.innerText = element.invested
                    .toLocaleString("pt-BR", { maximumFractionDigits: 2, minimumFractionDigits: 2 })
                const acumulated = document.createElement("td")
                acumulated.innerText = element.acumulated
                    .toLocaleString("pt-BR", { maximumFractionDigits: 2, minimumFractionDigits: 2 })
                const increase = document.createElement("td")
                increase.innerText = element.increase
                    .toLocaleString("pt-BR", { maximumFractionDigits: 2, minimumFractionDigits: 2 })
                const increaseInterest = document.createElement("td")
                increaseInterest.innerText = element.increase_interest
                    .toLocaleString("pt-BR", { maximumFractionDigits: 2, minimumFractionDigits: 2 })

                row.appendChild(month)
                row.appendChild(invested)
                row.appendChild(acumulated)
                row.appendChild(increase)
                row.appendChild(increaseInterest)

                resultsTable.appendChild(row)
            });

            document.getElementsByTagName("container")[0].appendChild(resultsTable)
        })
}
Chart.defaults.color = "#fff"
Chart.defaults.devicePixelRatio = 2
Chart.defaults.font.size = 14
function createGraph(data) {
    const labels = data.map(item => {
        if (item.month < 12) {
            return item.month + "m"
        }
        else if (item.month % 12 == 0) {
            return `${Math.floor(item.month / 12)}a`
        }
        else {
            return `${Math.floor(item.month / 12)}a ${item.month % 12}m`
        }
    })
    const investedData = data.map(item => item.invested)
    const accumulatedData = data.map(item => item.acumulated)

    const ctx = document.getElementById('myChart')
    new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Investido',
                    data: investedData,
                    borderColor: '#FF638460',
                    backgroundColor: '#FF6384FF',
                    tension: 0.4
                },
                {
                    label: 'Acumulado',
                    data: accumulatedData,
                    borderColor: '#36A2EB60',
                    backgroundColor: '#36A2EBFF',
                    tension: 0.4
                }
            ]
        },
        options: {
            responsive: true,
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'Tempo'
                    },
                    grid: {
                        color: '#ffffff20'
                    },
                },
                y: {
                    title: {
                        display: true,
                        text: 'Valor ($)'
                    },
                    grid: {
                        color: '#ffffff20'
                    },
                    beginAtZero: true
                }
            }
        }
    }
    )
    setTimeout(() => { window.scrollTo({ top: 500, behavior: 'smooth' }) }, 100)
}

document.getElementById("initial").addEventListener("input", (event) => {check_inputs()})
document.getElementById("interest").addEventListener("input", (event) => {check_inputs()})
document.getElementById("time").addEventListener("input", (event) => {check_inputs()})
document.getElementById("contribution").addEventListener("input", (event) => {check_inputs()})

function check_inputs() {
    removeTableChart()

    const initial = parseFloat(document.getElementById("initial").value)
    const interest = parseFloat(document.getElementById("interest").value) / 100
    const time = parseFloat(document.getElementById("time").value)

    if (isNaN(initial) | isNaN(interest) || isNaN(time) ) {
        document.getElementById("calculate").disabled = true
        document.getElementById("csv").disabled = true
    } else {
        document.getElementById("calculate").disabled = false
        document.getElementById("csv").disabled = false
    }
}

function csv() {
    const initial = parseFloat(document.getElementById("initial").value.replace(",", "."))
    const interest = parseFloat(document.getElementById("interest").value.replace(",", ".")) / 100
    const time = parseFloat(document.getElementById("time").value)
    const contribution = parseFloat(document.getElementById("contribution").value.replace(",", "."))

    const interest_unit = document.querySelector('input[name="interest_unit"]:checked').value;
    const time_unit = document.querySelector('input[name="time_unit"]:checked').value;

    const params = new URLSearchParams(
        {
            initial: initial,
            interest: interest,
            interest_unit: interest_unit,
            time: time,
            time_unit: time_unit,
            contribution: contribution ? contribution : 0.0,
        })

    download(`/juro/csv?${params.toString()}`, "res.csv");
}

function download(dataurl, filename) {
    const link = document.createElement("a");
    link.href = dataurl;
    link.download = filename;
    link.click();
  }

function removeTableChart() {
    let chartStatus = Chart.getChart("myChart")
    if (chartStatus != undefined) {
        chartStatus.destroy()
    }
    const resultsTableOld = document.getElementById("results-table")
    if (resultsTableOld !== null) {
        resultsTableOld.remove()
    }
}
  


