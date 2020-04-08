var myApp = angular.module('myApp', ['ngRoute'])

.controller('MainController', function($scope, $route, $routeParams, $location) {
    $scope.$route = $route;
    $scope.$location = $location;
    $scope.$routeParams = $routeParams;
})

.controller('JobsController', function($scope, $routeParams, $http) {
    selectedJobs();
    
    $scope.name = 'JobsController';
    $scope.params = $routeParams;

    $scope.active_page = "jobs";

    $http.get("/api/job/list")
        .then(function (response) {
            $scope.jobs = response.data;
        });
})

.controller('MachinesController', function($scope, $routeParams, $http) {
    selectedMachines();

    $scope.name = 'MachinesController';
    $scope.params = $routeParams;

    $scope.active_page = "machines";

    $scope.machines = null;
    $scope.serialized = JSON.stringify($scope.machines, null, 2);

    $http.get("/api/machine/list")
        .then(function (response) {
            $scope.machines = response.data;
        });
})

.controller('UsersController', function($scope, $routeParams, $http) {
    selectedUsers();

    $scope.name = 'UsersController';
    $scope.params = $routeParams;

    $scope.active_page = "users";

    $http.get("/api/user/list")
        .then(function (response) {
            $scope.users = response.data;
        });
})

.controller('LogController', function($scope, $routeParams, $http) {
    selectedLog();
    
    $scope.name = 'LogController';
    $scope.params = $routeParams;

    $scope.active_page = "log";
})

.config(function($routeProvider, $locationProvider) {
    $routeProvider
        .when('/jobs/', {
            templateUrl: 'jobs.html',
            controller: 'JobsController'
        })
        .when('/machines/', {
            templateUrl: 'machines.html',
            controller: 'MachinesController'
        })
        .when('/users/', {
            templateUrl: 'users.html',
            controller: 'UsersController'
        })
        .when('/log/', {
            templateUrl: 'log.html',
            controller: 'LogController'
        })
        .otherwise({
            redirectTo: "/jobs/"
        })

   $locationProvider.html5Mode(true);
});



let selectedJobs = function() {
    hideAllPointers();
    document.getElementById("jobs_pointer").style.display = "inline";
}

let selectedMachines = function() {
    hideAllPointers();
    document.getElementById("machines_pointer").style.display = "inline";
}

let selectedUsers = function() {
    hideAllPointers();
    document.getElementById("users_pointer").style.display = "inline";
}

let selectedLog = function() {
    hideAllPointers();
    document.getElementById("log_pointer").style.display = "inline";
}

let hideAllPointers = function() {
    document.getElementById("jobs_pointer").style.display = "none";
    document.getElementById("machines_pointer").style.display = "none";
    document.getElementById("users_pointer").style.display = "none";
    document.getElementById("log_pointer").style.display = "none";
}

/*
document.addEventListener('DOMContentLoaded', init, false);
function init() { }
*/

// This function receives ID of modal-ready div,
//  displays it as a block, and registers event handler
//  to dispose this modal.
let showModal = function(modalId) {
    // Get the modal
    var modal = document.getElementById(modalId);

    // Get the <span> element that closes the modal
    var span = document.getElementsByClassName("close")[0];

    modal.style.display = "block";

    // When the user clicks on <span> (x), close the modal
    span.onclick = function() {
        modal.style.display = "none";
    }

    // When the user clicks anywhere outside of the modal, close it
    window.onclick = function(event) {
        if (event.target == modal) {
            modal.style.display = "none";
        }
    }
}