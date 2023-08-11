from astron.object_repository import InterestInternalRepository
from time import sleep
from globals import *


class Services:
    def __init__(self):
        self.ir = InterestInternalRepository(DC_FILE, SSChannel, 0, ServicesChannel)
        self.ir.connect(self.connection_success, self.connection_failure,
                        host=MD_HOST, port=MD_PORT)

    def connection_success(self):
        print("Connection success!")

        self.ir.create_distobjglobal_view("AnonymousContactUD", AnonymousContactID, set_ai=True)
        self.ir.create_distobj("RootAI", RootID, 0, 0, set_ai=True)
        self.ir.create_distobj("LoginManagerAI", LoginManagerId, RootID, LOGIN_ZONE, set_ai=True)
        self.ir.create_distobj("DistributedWorldAI", DistributedWorldId, RootID, WORLD_ZONE, set_ai=True)

        while True:
            self.ir.poll_till_empty()
            for i in range(len(AI_TASKS)):
                AI_TASKS[i]()  # execute tasks per server frame
            sleep(1.0 / float(AI_FRAME_RATE))

    def connection_failure(self):
        print("Connection failure! Is the Message Director up?")
        return  # TODO: Handle event (libastron.python does not handle failure either!)


server = Services()
