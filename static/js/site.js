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

    $scope.addJob = function() {
        let id = uuidv4();
        $scope.jobs.push({
            "id": id,
            "name": "New job",
            "schedule": "* * * * *",
            "code": "",
            "send_email": false,
            "last_status": false,
            "target": {
                "name": "",
                "username": "",
                "url": "",
                "port": 22
            },
            "owner": {
                "name": "",
                "email": ""
            }
        });
        $scope.showModal = id;
    }

    $scope.deleteJob = function(id) {
        $scope.jobs = $scope.jobs.filter(function(job) {
            return job.id !== id;
        });
    }

    $scope.updateJobs = function() {
        let data = JSON.stringify($scope.jobs);
        console.log(data);
        $http.post("/api/job/bulk_update", data)
            .then(function() {
                console.log("success");
                $scope.refresh();
            }, function() {
                console.log("failure");
            });
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

    $scope.machines = [];
    $scope.machinesOld = [];

    $scope.refresh = function() {
        $http.get("/api/machine/list")
            .then(function (response) {
                $scope.machines = response.data;
                $scope.machinesOld = JSON.parse(JSON.stringify($scope.machines));
            });
    }

    $scope.addMachine = function() {
        $scope.machines.push({
            "id": "00000000-0000-0000-0000-000000000000",
            "name": "New machine"
        });
    }

    $scope.deleteMachine = function(id) {
        $scope.machines = $scope.machines.filter(function(machine) {
            return machine.id !== id;
        });
    }

    $scope.hasChanged = function() {
        let newMachines = $scope.machines;
        let newMachinesLen = newMachines.length; 
        let oldMachines = $scope.machinesOld;
        let oldMachinesLen = oldMachines.length;
    
        if (newMachinesLen !== oldMachinesLen) {
            return true;
        }
    
        for (var ix = 0; ix < newMachinesLen; ix++) {
            let newMachine = newMachines[ix];
            let oldMachine = oldMachines[ix];
    
            if (newMachine.id !== oldMachine.id ||
                newMachine.name !== oldMachine.name ||
                newMachine.username !== oldMachine.username ||
                newMachine.url !== oldMachine.url ||
                newMachine.port !== oldMachine.port)
            {
                return true;
            }
        }
    
        return false;
    }

    $scope.refresh();
})

.controller('UsersController', function($scope, $routeParams, $http) {
    selectedUsers();

    $scope.name = 'UsersController';
    $scope.params = $routeParams;

    $scope.active_page = "users";

    $scope.users = [];
    $scope.usersOld = [];

    $scope.refresh = function() {
        $http.get("/api/user/list")
            .then(function (response) {
                $scope.users = response.data;
                $scope.usersOld = JSON.parse(JSON.stringify($scope.users));
            });
    }

    $scope.addUser = function() {
        $scope.users.push({
            "id": "00000000-0000-0000-0000-000000000000",
            "name": "New user",
            "email": ""
        });
    }

    $scope.deleteUser = function(id) {
        $scope.users = $scope.users.filter(function(user) {
            return user.id !== id;
        });
    }

    $scope.hasChanged = function() {
        let newUsers = $scope.users;
        let newUsersLen = newUsers.length; 
        let oldUsers = $scope.usersOld;
        let oldUsersLen = oldUsers.length;
    
        if (newUsersLen !== oldUsersLen) {
            return true;
        }
    
        for (var ix = 0; ix < newUsersLen; ix++) {
            let newUser = newUsers[ix];
            let oldUser = oldUsers[ix];
    
            if (newUser.id !== oldUser.id ||
                newUser.name !== oldUser.name ||
                newUser.email !== oldUser.email)
            {
                return true;
            }
        }
    
        return false;
    }

    $scope.refresh();
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