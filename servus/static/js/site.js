document.addEventListener("DOMContentLoaded", init, false);

function init() {
    selectedJobs();
}

function selectedJobs() {
    hideAllPointers();
    document.getElementById("jobs_pointer").style.display = "inline";
}

function selectedMachines() {
    hideAllPointers();
    document.getElementById("machines_pointer").style.display = "inline";
}

function selectedUsers() {
    hideAllPointers();
    document.getElementById("users_pointer").style.display = "inline";

    performRequest("GET", "/api/user/list",
        function(result) {
            // console.log(result);

            let users = JSON.parse(result);
            for (let i = 0; i < users.length; i++) {
                let user = users[i];
                console.log(`USER [${user.id}] Name: ${user.name} Email: ${user.email}`);

            }


            createTable(users);
        });
}

function selectedLog() {
    hideAllPointers();
    document.getElementById("log_pointer").style.display = "inline";
}

let hideAllPointers = function() {
    document.getElementById("jobs_pointer").style.display = "none";
    document.getElementById("machines_pointer").style.display = "none";
    document.getElementById("users_pointer").style.display = "none";
    document.getElementById("log_pointer").style.display = "none";
}

let performRequest = function(httpMethod, url, successFn, errorFn) {
    let xmlhttp = new XMLHttpRequest();

    xmlhttp.onreadystatechange = function() {
        if (xmlhttp.readyState == XMLHttpRequest.DONE) {   // XMLHttpRequest.DONE == 4
           if (xmlhttp.status == 200) {
               successFn(xmlhttp.responseText);
           }
           else {
               console.error(`Error ${httpMethod} on ${url}.`)
               errorFn();
           }
        }
    };

    xmlhttp.open(httpMethod, url, true);
    xmlhttp.send();
}

let createTable = function(data) {
    /*
    var btn = document.createElement("BUTTON");   // Create a <button> element
    btn.innerHTML = "CLICK ME";                   // Insert text
    document.body.appendChild(btn); 
    */

    /*
    <table>

      {{#each days}}

      <tr>
        <td class="col_date">{{day}}. {{../../month}}.</td>
        <td class="col_weekday {{~#if is_non_workday}} non_working_day {{~/if~}}">{{weekday}}</td>
        <td class="col_event">
          <input type="text" data-day="{{day}}" value="{{event}}"/>
        </td>
      </tr>

      {{/each}}

    </table>
    */

    let table = document.createElement("table");

    for (let i = 0; i < data.length; i++) {
        let row = data[i];

        let tr = document.createElement("tr");

        let td = document.createElement("td");
        td.innerHTML = "user";


        tr.appendChild(td)
        table.appendChild(tr);
    }

    document.getElementById("main").appendChild(table);
}