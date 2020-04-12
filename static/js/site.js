var servus = angular.module('servus', ['ngRoute'])

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

    $scope.jobs = [];
    $scope.jobsOld = [];

    $scope.refresh = function() {

        $http.get("/api/job/list")
            .then(function (response) {
                $scope.jobs = response.data;
                $scope.jobsOld = JSON.parse(JSON.stringify($scope.jobs));
            });

        $http.get("/api/user/list")
            .then(function (response) {
                $scope.users = response.data;
            });

        $http.get("/api/machine/list")
            .then(function (response) {
                $scope.machines = response.data;
            });
    }

    $scope.temp_id = 0;

    $scope.addJob = function() {
        $scope.temp_id = $scope.temp_id + 1;
        $scope.jobs.push({
            //"id": `temporary-id-${$scope.temp_id}`,
            "id": "00000000-0000-0000-0000-000000000000",
            "name": "New job",
            "schedule": "* * * * *"
        });
    }

    $scope.deleteJob = function(id) {
        $scope.jobs = $scope.jobs.filter(function(job) {
            return job.id !== id;
        });
    }

    $scope.updateJobs = function() {
        console.log("update jobs")
        let data = JSON.stringify($scope.jobs);
        $http.post("/api/job/bulk_update", data)
            .then($scope.refresh());
    }
    
    $scope.hasChanged = function() {
        let newJobs = $scope.jobs;
        let newJobsLen = newJobs.length; 
        let oldJobs = $scope.jobsOld;
        let oldJobsLen = oldJobs.length;

        if (newJobsLen !== oldJobsLen) {
            return true;
        }

        for (var ix = 0; ix < newJobsLen; ix++) {
            let newJob = newJobs[ix];
            let oldJob = oldJobs[ix];

            if (newJob.id !== oldJob.id ||
                newJob.name !== oldJob.name ||
                newJob.description !== oldJob.description ||
                newJob.schedule !== oldJob.schedule ||
                newJob.target.id !== oldJob.target.id ||
                newJob.owner.id !== oldJob.owner.id ||
                newJob.send_email !== oldJob.send_email ||
                newJob.code !== oldJob.code)
            {
                return true;
            }
        }

        return false;
    }

    $scope.refresh();
})

.controller('MachinesController', function($scope, $routeParams, $http) {
    selectedMachines();

    $scope.name = 'MachinesController';
    $scope.params = $routeParams;

    $scope.active_page = "machines";

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

    $http.get("/api/log/0/20")
        .then(function (response) {
            $scope.log = response.data;
        });
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