pipeline {
    agent any
    stages {
        stage('MegaLinter') {
            agent {
                docker {
                    image 'megalinter/megalinter:v5'
                    args "-u root -e VALIDATE_ALL_CODEBASE=true -v ${WORKSPACE}:/tmp/lint --entrypoint=''"
                    reuseNode true
                }
            }
            steps {
                sh '/entrypoint.sh'
            }
        }
    }
}