pipeline {
    agent any
    stages {
        stage('Stage 1') {
            steps{
                def scannerHome = tool 'sonar';
                withSonarQubeEnv('SonarCloud') { // If you have configured more than one global server connection, you can specify its name
                    sh "${scannerHome}/bin/sonar-scanner"
                }
            }
        }
    }
}